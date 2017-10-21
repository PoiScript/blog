import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { RouterTestingModule } from '@angular/router/testing';

import { LoadingService } from 'app/core';
import { SharedModule } from 'app/shared';

import { NavComponent } from './nav.component';

describe('NavComponent', () => {
  let component: NavComponent;
  let element: HTMLElement;
  let fixture: ComponentFixture<NavComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      imports: [SharedModule, RouterTestingModule],
      declarations: [NavComponent],
      providers: [LoadingService]
    })
      .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(NavComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement.nativeElement;
    fixture.detectChanges();
  });

  it('should display a router link to homepage', () => {
    const solomon = element.querySelector('a:nth-child(1)');
    expect(solomon.textContent).toContain('SOLOMON');
    expect(solomon.getAttribute('routerLink')).toBe('/');
  });

  it('should display a router link to about page', () => {
    const about = element.querySelector('a:nth-child(2)');
    expect(about.textContent).toContain('ABOUT');
    expect(about.getAttribute('routerLink')).toBe('/about');
  });

  it('should display a router link to link page', () => {
    const link = element.querySelector('a:nth-child(3)');
    expect(link.textContent).toContain('LINK');
    expect(link .getAttribute('routerLink')).toBe('/link');
  });
});
