import { WebviewWindow } from '@tauri-apps/api/window'
import { Command } from '@tauri-apps/api/shell';

export const NewWindow = ({ title, url, wid }) => {
    const webview = new WebviewWindow(wid, {
        title: title,
        focus: true,
        skipTaskbar: true,
        width: 1200,
        height: 800,
        url
    })
    webview.once('tauri://created', function () {
        console.log("tauri://created")
    })
}

export const OpenBySystem = async ({ Path }) => {
    console.log('OpenBySystem', Path)
    const command = new Command('startCMD', [Path])
    const child = await command.spawn();
    console.log('pid:', child.pid);
}