import {ChangeDetectorRef, Component, ElementRef, HostListener, Input, ViewChild} from '@angular/core';
import {Wallpaper} from "../../modal/wallpaper";
import {DropDownService} from "../../service/drop-down.service";
import {MessageService} from "../../service/message.service";
import {appWindow} from "@tauri-apps/api/window";
import {Payload} from "../../modal/payload";
import {invoke} from "@tauri-apps/api";

@Component({
  selector: 'app-wallpaper-view',
  templateUrl: './wallpaper-view.component.html',
  styleUrls: ['./wallpaper-view.component.css']
})
export class WallpaperViewComponent {

  @Input({required: true}) source!: string;

  @ViewChild('wallpaper')
  wallpaperDiv!: ElementRef;

  wallpaper_list: Array<Wallpaper> = [];

  isLoadingEnd: boolean = false;

  currentPage = 1;

  havePage = true;

  constructor(private dropDownService: DropDownService, public changeDetectorRef:ChangeDetectorRef) {

  }

  async ngOnInit() {
    
    window.addEventListener("contextmenu", (e) => {
      e.preventDefault();
    });
    await this.onHomeRefresh();

    await appWindow.listen('download_progress', (data) => {
      const payload = data.payload as Payload;
      this.wallpaper_list.forEach(wallpaper => {
        if(payload.id === wallpaper.id) {
          wallpaper.process = payload.process;
          wallpaper.text = payload.text;
          this.changeDetectorRef.detectChanges();
        }
      })
    });

    await appWindow.listen('bing_refresh_finished', async (data) => {
      await this.onHomeRefresh();
    });

    await appWindow.listen('spotlight_refresh_finished', async (data) => {
      await this.onHomeRefresh();
    });
  }

  async onHomeRefresh(){
    this.currentPage = 1;
    const wallpaper_list = await this.getWallpaper(this.currentPage);
    console.log(wallpaper_list);
    if(wallpaper_list.length>0){
      const wallpaperDiv = this.wallpaperDiv.nativeElement as HTMLDivElement;
      wallpaperDiv.scrollTop = 0;
      this.wallpaper_list = wallpaper_list;
      this.isLoadingEnd = false;
      this.havePage = true;
      this.changeDetectorRef.detectChanges();
    }
  }

  async getWallpaper(currentPage: number): Promise<Array<Wallpaper>> {
    console.log(this.source);
    return await invoke<Array<Wallpaper>>("get_wallpaper", {currentPage: currentPage, source: this.source});
  }



  async setWallpaper(currentPage: number) {
    const wallpaper_list = await this.getWallpaper(currentPage);
    console.log(wallpaper_list);
    if (wallpaper_list.length > 0) {
      this.wallpaper_list.push(...wallpaper_list);
    } else {
      this.havePage = false;
    }
  }

  async img_click(wallpaper: Wallpaper) {
    if(wallpaper.disabled){
      return
    }
    wallpaper.disabled = true;
    wallpaper.process = 0;
    wallpaper.text = "设置壁纸中"
    await invoke("set_wallpaper", { wallpaper });
    wallpaper.text = ""
    wallpaper.disabled = false
  }

  sleep = async (waitTime: number) =>
      new Promise(resolve =>
          setTimeout(resolve, waitTime));

  //滑动加载[下拉翻页]
  @HostListener('window:scroll', ['$event'])
  async onScroll(event: Event) {
    if (this.havePage) {
      const isEnd = this.dropDownService.calculation(event);
      if (isEnd && !this.isLoadingEnd) {
        this.isLoadingEnd = true;
        this.currentPage = this.currentPage + 1;
        await this.sleep(100);
        await this.setWallpaper(this.currentPage);
        this.isLoadingEnd = false;
      }
    }
  }

  getNormalImgUrl(wallpaper: Wallpaper): string {
    return "asset://localhost/" +wallpaper.normal_file_path;
  }


  protected readonly Math = Math;
}
