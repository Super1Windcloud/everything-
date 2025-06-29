import {TrayIcon, TrayIconOptions} from "@tauri-apps/api/tray";
import {defaultWindowIcon} from "@tauri-apps/api/app";
import {Menu} from "@tauri-apps/api/menu/menu";


const menu = await Menu.new({
    items: [
        {
            id: 'quit',
            text: 'Quit',
            action: () => {
                console.log('quit pressed');
            },
        },

        {
            id: 'options'
            , text: 'Options',
            action: () => {
                console.log('options pressed');
            }
        },
        {
            id: 'openFileList',
            text: 'Open File List',
            action: () => {
                console.log('openFileList pressed');
            }
        }
        , {
            id: 'showSearchWindow'
            , text: 'Show Search Window',
            action: () => {
                console.log('showSearchWindow pressed');
            }
        }
    ],
});


const options: TrayIconOptions = {
    icon: (await defaultWindowIcon()) ?? undefined,
    menu,
    showMenuOnLeftClick: true,
};

export async function initTrayIcon() {
    await TrayIcon.new(options);
}





