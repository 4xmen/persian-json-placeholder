<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;

/*
|--------------------------------------------------------------------------
| API Routes
|--------------------------------------------------------------------------
|
| Here is where you can register API routes for your application. These
| routes are loaded by the RouteServiceProvider within a group which
| is assigned the "api" middleware group. Enjoy building your API!
|
*/

Route::middleware('auth:sanctum')->get('/user', function (Request $request) {
    return $request->user();
});
Route::get('/',function (){
   return [
       'message' => 'welcome to json placeholderr [Persian]',
       'v1' => 'available'
       ];
});

Route::prefix('v1')->name('v1.')->group(
    function () {
        Route::get('/',function (){
            return [
                'message' => 'welcome v1 to json placeholderr [Persian]',
            ];
        });

        Route::get('posts', [\App\Http\Controllers\PostController::class,"index"])
            ->name('posts');
        Route::get('post/{post}', [\App\Http\Controllers\PostController::class,"show"])
            ->name('post');

        Route::get('users', [\App\Http\Controllers\UserController::class,"index"])
            ->name('users');
        Route::get('user/{user}', [\App\Http\Controllers\UserController::class,"show"])
            ->name('user');

        Route::get('products', [\App\Http\Controllers\ProductController::class,"index"])
            ->name('products');
        Route::get('product/{product}', [\App\Http\Controllers\ProductController::class,"show"])
            ->name('product');

        Route::get('categories', [\App\Http\Controllers\CategoryController::class,"index"])
            ->name('categories');
        Route::get('category/{category}', [\App\Http\Controllers\CategoryController::class,"show"])
            ->name('category');

        Route::get('comments', [\App\Http\Controllers\CommentController::class,"index"])
            ->name('comments');

        Route::get('comment/{comment}', [\App\Http\Controllers\CommentController::class,"show"])
            ->name('comment');


    }
);
