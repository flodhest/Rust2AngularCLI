import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError, map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class GridService {
  private baseUrl = 'https://localhost:7114/api';

  constructor(private http: HttpClient) {}

  saveGrid(gridToSave: { name: string; data: string[][] }): Observable<any> {
    const dataJsonString = JSON.stringify(gridToSave.data);
    const gridData = {
      name: gridToSave.name,
      data: dataJsonString,
    };

    return this.http
      .post<any>(`${this.baseUrl}`, gridData, {
        withCredentials: false,
      })
      .pipe(
        catchError((error) => {
          console.error('Error saving grid:', error);
          return throwError(error);
        })
      );
  }

  getSavedGrids(): Observable<any[]> {
    return this.http.get<any[]>(`${this.baseUrl}`).pipe(
      catchError((error) => {
        console.error('Error getting saved grids:', error);
        return throwError(error);
      })
    );
  }

  getGridList(): Observable<any[]> {
    return this.http.get<any[]>(`${this.baseUrl}`).pipe(
      catchError((error) => {
        console.error('Error getting grid list:', error);
        return throwError(error);
      })
    );
  }

  loadGrid(gridId: number): Observable<{ name: string; data: string[][] }> {
    return this.http
      .get<{ name: string; data: string[][] }>(`${this.baseUrl}/${gridId}`)
      .pipe(
        catchError((error) => {
          console.error('Error loading grid:', error);
          return throwError(error);
        })
      );
  }

  deleteGrid(gridId: number): Observable<any> {
    return this.http.delete<any>(`${this.baseUrl}/${gridId}`).pipe(
      catchError((error) => {
        console.error('Error deleting grid:', error);
        return throwError(error);
      })
    );
  }
}
