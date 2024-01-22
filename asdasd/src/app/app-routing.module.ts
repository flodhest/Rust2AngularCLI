// app-routing.module.ts

import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HomeComponent } from './components/Home/Home.component';
import { PlaceholderComponent1 } from './components/PlaceholderComponent1/PlaceholderComponent1.component';
import { PlaceholderComponent2 } from './components/PlaceholderComponent2/PlaceholderComponent2.component';

const routes: Routes = [
  { path: 'home', component: HomeComponent },
  { path: 'placeholder-component1', component: PlaceholderComponent1 },
  { path: 'placeholder-component2', component: PlaceholderComponent2 },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
