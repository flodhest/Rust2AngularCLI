// app.module.ts
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { FormsModule } from '@angular/forms';
import { AppRoutingModule } from './app-routing.module';
import { HttpClientModule } from '@angular/common/http';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatIconModule } from '@angular/material/icon';
import { MatSnackBarModule } from '@angular/material/snack-bar';
import { MatInputModule } from '@angular/material/input';
import { ReactiveFormsModule } from '@angular/forms';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatListModule } from '@angular/material/list';
import { MatButtonModule } from '@angular/material/button';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatSelectModule } from '@angular/material/select';
import { ServiceWorkerModule } from '@angular/service-worker';

import { AppComponent } from './app.component';
import { HomeComponent } from './components/Home/Home.component';
import { PlaceholderComponent1 } from './components/PlaceholderComponent1/PlaceholderComponent1.component';
import { PlaceholderComponent2 } from './components/PlaceholderComponent2/PlaceholderComponent2.component';

@NgModule({
  declarations: [
    AppComponent,
    HomeComponent,
    PlaceholderComponent1,
    PlaceholderComponent2
  ],
  imports: [
    BrowserModule,
    FormsModule,
    AppRoutingModule,
    HttpClientModule,
    MatFormFieldModule,
    MatIconModule,
    MatSnackBarModule,
    MatInputModule,
    ReactiveFormsModule,
    BrowserAnimationsModule,
    MatListModule,
    MatButtonModule,
    MatToolbarModule,
    MatSelectModule,
    ServiceWorkerModule.register('ngsw-worker.js', { enabled: true })
  ],
  bootstrap: [AppComponent]
})
export class AppModule {}
