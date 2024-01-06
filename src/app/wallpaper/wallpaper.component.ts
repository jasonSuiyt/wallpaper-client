import {Component, Input} from '@angular/core';
import {Wallpaper} from "../modal/wallpaper";

@Component({
  selector: 'app-wallpaper',
  templateUrl: './wallpaper.component.html',
  styleUrls: ['./wallpaper.component.css']
})
export class WallpaperComponent {

  @Input()
  wallpaper_list: Array<Wallpaper> = [];

}
