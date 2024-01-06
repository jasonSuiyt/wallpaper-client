import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, Routes } from '@angular/router';
import { BingComponent } from './bing/bing.component';
import {MicrosoftComponent} from "./microsoft/microsoft.component";


const routes: Routes = [
  { path: '', redirectTo: '/bing', pathMatch: 'full' },
  { path: 'bing', component: BingComponent },
  { path: 'microsoft', component: MicrosoftComponent },
];

@NgModule({
  declarations: [],
  imports: [
    RouterModule.forRoot(routes),
    CommonModule
  ],
  exports: [
    RouterModule
  ]
})
export class AppRoutingModule { }
