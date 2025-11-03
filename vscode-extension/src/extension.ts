import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
    console.log('Jounce extension is now active!');

    // Start LSP client (will be implemented in Task 2)
    startLanguageClient(context);

    // Register commands
    registerCommands(context);

    // Set up format on save
    setupFormatOnSave(context);
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

function startLanguageClient(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('jounce');
    const jncPath = config.get<string>('lspPath', 'jnc');

    // Server options - spawn the 'jnc lsp' process
    const serverOptions: ServerOptions = {
        command: jncPath,
        args: ['lsp'],
        options: {
            cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath
        }
    };

    // Client options - configure the language client
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'jnc' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.jnc')
        }
    };

    // Create and start the language client
    client = new LanguageClient(
        'jounce',
        'Jounce Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client (this will also launch the server)
    client.start();

    // Show status in status bar
    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(check) Jounce';
    statusBarItem.tooltip = 'Jounce LSP Server Active';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);
}

function registerCommands(context: vscode.ExtensionContext) {
    // Command: Compile Current File
    context.subscriptions.push(
        vscode.commands.registerCommand('jounce.compile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'jnc') {
                vscode.window.showErrorMessage('No active .jnc file');
                return;
            }

            const filePath = editor.document.uri.fsPath;
            const config = vscode.workspace.getConfiguration('jounce');
            const jncPath = config.get<string>('lspPath', 'jnc');

            const terminal = vscode.window.createTerminal('Jounce Compile');
            terminal.show();
            terminal.sendText(`${jncPath} compile "${filePath}"`);
        })
    );

    // Command: Watch Current File
    context.subscriptions.push(
        vscode.commands.registerCommand('jounce.watch', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'jnc') {
                vscode.window.showErrorMessage('No active .jnc file');
                return;
            }

            const filePath = editor.document.uri.fsPath;
            const config = vscode.workspace.getConfiguration('jounce');
            const jncPath = config.get<string>('lspPath', 'jnc');

            const terminal = vscode.window.createTerminal('Jounce Watch');
            terminal.show();
            terminal.sendText(`${jncPath} watch "${filePath}"`);
        })
    );

    // Command: Format Document
    context.subscriptions.push(
        vscode.commands.registerCommand('jounce.format', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'jnc') {
                vscode.window.showErrorMessage('No active .jnc file');
                return;
            }

            await vscode.commands.executeCommand('editor.action.formatDocument');
        })
    );

    // Command: Run Tests
    context.subscriptions.push(
        vscode.commands.registerCommand('jounce.test', async () => {
            const terminal = vscode.window.createTerminal('Jounce Tests');
            terminal.show();
            terminal.sendText('cargo test');
        })
    );

    // Command: Show Profiling
    context.subscriptions.push(
        vscode.commands.registerCommand('jounce.profile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'jnc') {
                vscode.window.showErrorMessage('No active .jnc file');
                return;
            }

            const filePath = editor.document.uri.fsPath;
            const config = vscode.workspace.getConfiguration('jounce');
            const jncPath = config.get<string>('lspPath', 'jnc');

            const terminal = vscode.window.createTerminal('Jounce Profile');
            terminal.show();
            terminal.sendText(`${jncPath} compile "${filePath}" --profile`);
        })
    );
}

function setupFormatOnSave(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.workspace.onWillSaveTextDocument((event) => {
            const config = vscode.workspace.getConfiguration('jounce');
            const formatOnSave = config.get<boolean>('formatOnSave', false);

            if (event.document.languageId === 'jnc' && formatOnSave) {
                event.waitUntil(
                    vscode.commands.executeCommand('editor.action.formatDocument')
                );
            }
        })
    );
}
