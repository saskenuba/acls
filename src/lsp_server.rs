use std::path::PathBuf;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{InitializeParams, InitializeResult, *};
use tower_lsp::{Client, LanguageServer, LspService, Server as LspServer};

#[derive(Debug)]
pub(crate) struct LspBackend {
    pub(crate) client: Client,
    pub(crate) world_state: WorldState,
}

#[derive(Debug, Default)]
struct WorldState {}

impl WorldState {
    fn snapshot(&self) -> WorldStateSnapshot {
        todo!()
    }
}

struct WorldStateSnapshot {}

#[tower_lsp::async_trait]
impl LanguageServer for LspBackend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let root_uri = params.root_uri;
        let folders = params.workspace_folders;

        let mut base_glob = root_uri.unwrap_or_else(|| folders.unwrap().remove(0).uri);

        let paths = glob_dir(&mut base_glob);

        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "server is shutting down gracefully.")
            .await;

        Ok(())
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let CodeActionParams {
            text_document,
            range,
            context,
            work_done_progress_params,
            partial_result_params,
        } = params;

        let action = CodeAction::default();
        CodeActionResponse::new().push(action.into());

        todo!()
    }

    async fn did_create_files(&self, params: CreateFilesParams) -> () {
        todo!()
    }
}

/// Returns a vector with all files found for ".clj, .cljs, and .cljc".
fn glob_dir(base_dir: &mut Url) -> Vec<PathBuf> {
    let base_dir = base_dir.to_string();
    let clj_glob = base_dir.to_owned() + "/**/*.clj";
    let cljs_glob = base_dir + "/**/*.cljs";

    let glob_iter = glob::glob(&clj_glob)
        .unwrap()
        .chain(glob::glob(&cljs_glob).unwrap())
        .flatten();

    let mut path_vec = Vec::new();
    for path in glob_iter {
        path_vec.push(path);
    }
    path_vec
}

impl LspBackend {
    pub async fn listen() {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
        let (mut stream, addr) = listener.accept().await.unwrap();
        println!("Setup server on port: {}", addr.port());

        let (r, w) = stream.split();
        let (service, socket) = LspService::new(|client| LspBackend {
            client,
            world_state: WorldState::default(),
        });
        LspServer::new(r, w, socket).serve(service).await;
    }
}
