import {animate, state, style, transition, trigger} from "@angular/animations";
import {Component, OnInit} from "@angular/core";

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.css"]
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
