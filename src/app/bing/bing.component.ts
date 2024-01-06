import {DropDownService} from '../service/drop-down.service';
import {ChangeDetectorRef, Component, ElementRef, HostListener, ViewChild} from '@angular/core';
import {invoke} from '@tauri-apps/api';
import {appWindow} from '@tauri-apps/api/window'
import {MessageService} from '../service/message.service';
import {MessageType} from '../enum/message-type';
import {Wallpaper} from "../modal/wallpaper";
import {Payload} from "../modal/payload";

@Component({
  selector: 'app-bing',
  templateUrl: './bing.component.html',
  styleUrls: ['./bing.component.css']
})
export class BingComponent {

}


