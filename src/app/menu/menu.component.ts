import {MessageService} from '../service/message.service';
import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {Router} from '@angular/router';
import {invoke} from '@tauri-apps/api';
import {animate, state, style, transition, trigger} from "@angular/animations";
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

@Component({
    selector: 'app-menu',
    templateUrl: './menu.component.html',
    styleUrls: ['./menu.component.css']
})
export class MenuComponent implements OnInit {

    constructor(private router: Router, private messageSrv: MessageService) {
    }

    ngOnInit(): void {

    }

    menuIndex = 0;

    @Output()
    menuClick: EventEmitter<String> = new EventEmitter();

    menuSelected: string = 'bing';


    async onMenuClick(_: MouseEvent, url: string, index: number) {
        this.menuIndex = index;
        this.menuSelected = url;
        // await this.router.navigate([url]);
        this.menuClick.emit(url);
    }

    async refreshClick($event: MouseEvent) {
        const refresh_icon = $event.target as HTMLOrSVGImageElement
        refresh_icon.classList.add('animate-spin');
        if(this.menuSelected == 'bing') {
            await invoke("refresh", {source: 'bing'});
        } else if (this.menuSelected == 'microsoft') {
            await invoke("refresh", {source: 'spotlight'});
        } else if (this.menuSelected == 'anime') {
            await invoke("refresh", {source: 'anime'});
        } else {
            await invoke("refresh", {source: 'wallpapers'});
        }
        refresh_icon.classList.remove('animate-spin');

        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }
        if (permissionGranted) {
            sendNotification({ title: '壁纸天堂', body: '壁纸更新完成' });
        }
    }

}
