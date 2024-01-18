import {animate, state, style, transition, trigger} from "@angular/animations";
import {Component, OnInit} from "@angular/core";

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.css"],
    animations: [
        trigger('bing', [
            state('anime', style({
                opacity: 0,
                zIndex : 0
            })),
            state('microsoft', style({
                opacity: 0,
                zIndex : 0
            })),
            state('wallpapers', style({
                opacity: 0,
                zIndex : 0
            })),
            state('bing', style({
                opacity: 1,
                zIndex : 2
            })),
            transition('* => bing', [
                animate('1s')
            ]),
        ]),
        trigger('microsoft', [
            state('anime', style({
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
            state('bing', style({
                opacity: 0,
                zIndex : 0
            })),
            transition('* => microsoft', [
                animate('1s')
            ]),
        ]),
        trigger('wallpapers', [
            state('anime', style({
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
            state('bing', style({
                opacity: 0,
                zIndex : 0
            })),
            transition('* => wallpapers', [
                animate('1s')
            ]),
        ]),
        trigger('anime', [
            state('anime', style({
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
            state('bing', style({
                opacity: 0,
                zIndex : 0
            })),
            transition('* => anime', [
                animate('1s')
            ]),
        ]),
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
