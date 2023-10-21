import { WebviewWindow ,getAll} from '@tauri-apps/api/window'
import { Command, open } from '@tauri-apps/api/shell';

export const NewWindow =async ({ title, url, wid }) => {
    const wins = getAll()
    const indexWindow=wins.findIndex(item=>item.label ==wid)
    if(indexWindow>=0){
      await  wins[indexWindow].close()
    }
    const webview = new WebviewWindow(wid, {
        title: title,
        focus: true,
        skipTaskbar: true,
        width: 1200,
        height: 850,
        url
    })
    webview.once('tauri://created', function () {
        console.log("tauri://created")
    })
}

export const CmdBySystem = async ({ Path }) => {
    console.log('OpenBySystem', Path)
    const command = new Command('startCMD', [Path])
    command.on('close', data => {
        console.log(`command finished with code ${data.code} and signal ${data.signal}`)
    });
    command.on('error', error => console.error(`command error: "${error}"`));
    command.stdout.on('data', line => console.log(`command stdout: "${line}"`));
    command.stderr.on('data', line => console.log(`command stderr: "${line}"`));
    command.spawn()
    

}

export const OpenBySystem = async ({ Path }) => {
    open("file://" + Path).then(res => {
        console.log('res', res)
    }).catch(err => {
        console.log('err', err)
    })
}