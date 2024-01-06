import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { AppRoutingModule } from './app-routing.module';
import { BingComponent } from './bing/bing.component';
import { MenuComponent } from "./menu/menu.component";
import { DropDownService } from "./service/drop-down.service";
import { MessageService } from "./service/message.service";
import {BrowserAnimationsModule} from "@angular/platform-browser/animations";
import { WallpaperComponent } from './wallpaper/wallpaper.component';
import { MicrosoftComponent } from './microsoft/microsoft.component';
import { WallpaperViewComponent } from './plugins/wallpaper-view/wallpaper-view.component';
import {NgOptimizedImage} from "@angular/common";
import {ScrollingModule} from "@angular/cdk/scrolling";

@NgModule({
  declarations: [AppComponent,MenuComponent, BingComponent, WallpaperComponent, MicrosoftComponent, WallpaperViewComponent],
  imports: [BrowserModule, AppRoutingModule, BrowserAnimationsModule, NgOptimizedImage, ScrollingModule],
  providers: [DropDownService, MessageService],
  bootstrap: [AppComponent],
})
export class AppModule {}
