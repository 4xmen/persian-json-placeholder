<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">

        <title>Laravel</title>

        <!-- Fonts -->
        <link href="https://cdn.jsdelivr.net/gh/rastikerdar/vazirmatn@v33.003/Vazirmatn-font-face.css" rel="stylesheet" type="text/css" />


        <style>
            * {
                font-family: 'Vazirmatn', sans-serif;
            }
            body{
                direction: rtl;
            }
            img{
                width: 250px;
                float: left;
            }
            code{
                direction: ltr;
                background: #e0e0e0;
                padding: 12px;
                display: block;
            }
            code a{
                color: red;
                text-decoration: none;
            }
            .container{
                max-width: 95%;
                margin: 2em auto;
            }
        </style>
    </head>
    <body>
        <section>
            <div class="container">
                <img src="{{asset('assets/xstack.png')}}" alt="">
                <h1>
                    Json placeholder persian - پارسی - فارسی
                </h1>
                <p>
                    سرویس فعلی برای آزمایش api بوجود آمده است تا برنامه‌نویسان گرامی برای تست برنامه های خود از api به صورت json استفاده کنند و زبان آن پارسی می‌باشد
                </p>
            </div>
        </section>
        <section>
            <div class="container">
                <h1>
                    API های مورد استفاده:
                </h1>

                <h3>
                    API RESTFull users (کاربران)
                </h3>
                <code>
                    <a href="{{route('v1.users')}}" target="_blank">
                        {{route('v1.users')}}
                    </a>
                </code>
                <h3>
                    API RESTFull user (کاربر)
                </h3>
                <code>
                    <a href="{{route('v1.user',1)}}"  target="_blank">
                        {{route('v1.user',1)}}
                    </a>
                </code>

                <hr>
                <h3>
                    API RESTFull posts (نوشته‌ها)
                </h3>
                <code>
                    <a href="{{route('v1.posts')}}"  target="_blank">
                        {{route('v1.posts')}}
                    </a>
                </code>
                <h3>
                    API RESTFull post (نوشته)
                </h3>
                <code>
                    <a href="{{route('v1.post',\App\Models\Post::limit(1)->first()->slug)}}"  target="_blank">
                        {{route('v1.post',\App\Models\Post::limit(1)->first()->slug)}}
                    </a>
                </code>

                <hr>
                <h3>
                    API RESTFull categories (دسته‌بندی‌ها)
                </h3>
                <code>
                    <a href="{{route('v1.categories')}}"  target="_blank">
                        {{route('v1.categories')}}
                    </a>
                </code>
                <h3>
                    API RESTFull category (دسته‌ها)
                </h3>
                <code>
                    <a href="{{route('v1.category',\App\Models\Category::orderByDesc('id')->limit(1)->first()->slug)}}"  target="_blank">
                        {{route('v1.category',\App\Models\Category::orderByDesc('id')->limit(1)->first()->slug)}}
                    </a>
                </code>

                <hr>
                <h3>
                    API RESTFull products (محصولات)
                </h3>
                <code>
                    <a href="{{route('v1.products')}}"  target="_blank">
                        {{route('v1.products')}}
                    </a>
                </code>
                <h3>
                    API RESTFull product (محصول)
                </h3>
                <code>
                    <a href="{{route('v1.product',\App\Models\Product::limit(1)->first()->slug)}}"  target="_blank">
                        {{route('v1.product',\App\Models\Product::limit(1)->first()->slug)}}
                    </a>
                </code>

                <hr>
                <h3>
                    API RESTFull products (دیدگاه‌ها|نظرات)
                </h3>
                <code>
                    <a href="{{route('v1.comments')}}"  target="_blank">
                        {{route('v1.comments')}}
                    </a>
                </code>
                <h3>
                    API RESTFull product (دیدگاه|نظر)
                </h3>
                <code>
                    <a href="{{route('v1.comment',\App\Models\Comment::limit(1)->first()->id)}}"  target="_blank">
                        {{route('v1.comment',\App\Models\Comment::limit(1)->first()->id)}}
                    </a>
                </code>

            </div>
        </section>


        <section>
            <div class="container">
                <h1>
                    گزینه‌ها
                </h1>
                <h3>
                    مرتب‌سازی | sort
                </h3>
                <code>
                    <a href="{{route('v1.products')}}?sort=view"  target="_blank">
                        {{route('v1.products')}}?sort=view
                    </a>
                </code>
                <hr>
                <h3>
                    نوع مرتب‌سازی | sortType
                </h3>
                <p>
                    مقداری بین asc,desc
                </p>
                <code>
                    <a href="{{route('v1.products')}}?sortType=desc"  target="_blank">
                        {{route('v1.products')}}?sortType=desc
                    </a>
                </code>

                <hr>

                <h3>
                    محدودیت صفحه
                </h3>
                <code>
                    <a href="{{route('v1.products')}}?limit=45"  target="_blank">
                        {{route('v1.products')}}?limit=45
                    </a>
                </code>
                <hr>

                <h3>
                    استفاده هم زمان
                </h3>
                <code>
                    <a href="{{route('v1.products')}}?limit=45&sort=sell_count&sortType=desc"  target="_blank">
                        {{route('v1.products')}}?limit=45&sort=sell_count&sortType=desc
                    </a>
                </code>
                <hr>
            </div>
        </section>
    </body>
</html>
