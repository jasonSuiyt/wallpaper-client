import { Injectable } from '@angular/core';
import { Subject } from 'rxjs';
import { Message } from '../modal/message';
import { MessageType } from '../enum/message-type';


@Injectable({
  providedIn: 'root'
})
export class MessageService {

  private sender = new Subject<Message>();

  constructor() { }

  send(message: Message) {
    message.id = this.getUniqueId(4);
    this.sender.next(message);
  }


  getUniqueId(parts: number): string {
    const stringArr = [];
    for (let i = 0; i < parts; i++) {
      const S4 = (((1 + Math.random()) * 0x10000) | 0).toString(16).substring(1);
      stringArr.push(S4);
    }
    return stringArr.join('-');
  }

  onMessage(callback: (message: Message) => void) {
    this.sender.subscribe(callback)
  }

}
