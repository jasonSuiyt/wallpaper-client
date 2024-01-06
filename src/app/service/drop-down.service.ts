import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class DropDownService {

  //1. 获取滚动条当前位置
  getScrollTop(div: HTMLDivElement): number {
    let scrollTop = 0;
    if (div && div.scrollTop) {
      scrollTop = div.scrollTop;
    } else if (document.body) {
      scrollTop = document.body.scrollTop;
    }
    return scrollTop;
  }
 
  //2. 获取当前可视范围高度
  getClientHeight(div: HTMLDivElement): number {
    return  div.clientHeight;
  }
 
// 获取文档完整的高度 = 1+2
  getScrollHeight(div: HTMLDivElement): number {
    return  div.scrollHeight;
  }
 
  calculation (event: Event) {
    const div  = event.target as HTMLDivElement;
    if (this.getScrollTop(div) + this.getClientHeight(div)
      >= this.getScrollHeight(div) ) {
      // console.log('触发');
      return true;
    } else {
      // console.log('没触发');
      return false;
    }
  }
}
