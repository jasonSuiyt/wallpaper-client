import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BingComponent } from './bing.component';

describe('BingComponent', () => {
  let component: BingComponent;
  let fixture: ComponentFixture<BingComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BingComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BingComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
