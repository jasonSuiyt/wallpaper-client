import {animate, state, style, transition, trigger} from "@angular/animations";
import {Component, OnInit} from "@angular/core";

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.css"],
    animations: [
        trigger('bing', [
            state('bing', style({
                opacity: 1,
                zIndex : 2
            })),
            state('microsoft', style({
                opacity: 0,
                zIndex : 0
            })),
            state('wallpapers', style({
                opacity: 0,
                zIndex : 0
            })),
            transition('microsoft => bing', [
                animate('1s')
            ]),
            transition('wallpapers => bing', [
                animate('1s')
            ]),
        ]),
        trigger('microsoft', [
            state('bing', style({
                opacity: 0,
                zIndex : 0
            })),
            state('microsoft', style({
                opacity: 1,
                zIndex : 2
            })),
            state('wallpapers', style({
                opacity: 0,
                zIndex : 0
            })),
            transition('bing => microsoft', [
                animate('1s')
            ]),
            transition('wallpapers => microsoft', [
                animate('1s')
            ]),
        ]),
        trigger('wallpapers', [
            state('bing', style({
                opacity: 0,
                zIndex : 0
            })),
            state('microsoft', style({
                opacity: 0,
                zIndex : 0
            })),
            state('wallpapers', style({
                opacity: 1,
                zIndex : 2
            })),
            transition('bing => wallpapers', [
                animate('1s')
            ]),
            transition('microsoft => wallpapers', [
                animate('1s')
            ]),
        ])
    ]
})
export class AppComponent implements OnInit {

    source = "bing";

    async ngOnInit(): Promise<void> {

    }

    menuClick($event: any): void {
        console.log($event);
        this.source = $event;
    }


}
