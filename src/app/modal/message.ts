import { MessageType } from "../enum/message-type";


export interface Message {
    msg: string;
    messageType: MessageType;
    id?: string;
}
