<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">

    <title>Persian json placeholder for developer - xstack</title>

    <!-- Fonts -->
    <link href="https://cdn.jsdelivr.net/gh/rastikerdar/vazirmatn@v33.003/Vazirmatn-font-face.css" rel="stylesheet"
          type="text/css"/>
    <style>
        * {
            padding: 0;
            margin: 0;
            box-sizing: border-box;
        }

        body {
            background-color: #f5f5f5;
            font-family: 'Vazirmatn', sans-serif;
            direction: rtl;
        }

        header {
            height: 100vh;
            overflow: hidden;
            position: relative;
            background-color: #f5f5f5;
        }

        code {
            direction: ltr;
            background: #e0e0e0bb;
            padding: 12px;
            display: block;
            transition: 450ms;
        }

        code:hover {
            background: #e0e0e0ff;
        }

        code a {
            color: red;
            text-decoration: none;
            margin: 8px 0;
        }

        p {
            text-align: justify;
        }

        .container {
            max-width: 95%;
            margin: 2em auto;
        }

        header .description {
            text-align: center;
            position: absolute;
            top: 5%;
            left: 50%;
            transform: translate(-50%, 0);
            z-index: 99;
        }

        header p, header h1 {
            color: #fff;
        }

        .title {
            margin: 25px 0;
            font-size: 2rem;
        }

        .card {
            width: 95%;
            margin: 25px auto;
            padding: 15px;
            background-color: #282c34;
            border-radius: 8px;
            z-index: 1;
            color: #fff;
        }

        .tools {
            display: flex;
            align-items: center;
            padding: 9px;
        }

        .circle {
            padding: 0 4px;
        }

        .box {
            display: inline-block;
            align-items: center;
            width: 10px;
            height: 10px;
            padding: 1px;
            border-radius: 50%;
        }

        .red {
            background-color: #ff605c;
        }

        .yellow {
            background-color: #ffbd44;
        }

        .green {
            background-color: #00ca4e;
        }

        .row {
            display: flex;
            flex-direction: row;
            justify-content: space-around;
        }

        .row img {
            width: 600px;

        }

        .row p {
            width: 80%;
            margin-top: 15px;
        }

        .bg {
            width: 100%;
            height: 100vh;
            background: #f5f5f5;
            display: block;
            z-index: 1;
            object-fit: fill;
        }

        /*-1000px width*/
        @media ( max-width: 1000px ) {
            .bg{
                width: auto ;
                height: 100vh ;
            }
            .row{
                display: block;
                text-align: center;
            }
            p{
                margin-bottom: 2em;
                width: 100% !important;
            }

            section img{
                max-width: 100%;
            }
        }

    </style>
</head>
<body>
<header>
    <div class="description">
        <img style="width: 250px;margin-bottom: 15%;" src="{{asset('assets/xstack.png')}}" alt="">
        <h1>
            Json placeholder persian - (پارسی | فارسی)
        </h1>
        <h2>
            WEB API آزمایشی برای برنامه نویسان پارسی زبان
        </h2>
        <p>
            سرویس فعلی برای آزمایش api بوجود آمده است تا برنامه‌نویسان گرامی برای تست برنامه های خود از api به صورت json
            استفاده کنند و زبان آن پارسی می‌باشد
        </p>
    </div>
    <img class="bg" src="{{asset('assets/bg.svg')}}" alt="">
</header>

<section>
    <div class="container">
        <div class="row">
            <div>
                <h3>مقدمه</h3>
                <p>
                    یکی از ابزار آموزشی برای برنامه‌نویسان Front-end، برنامه‌نویسان موبایل و یادگیری کار با رست‌فول
                    ای‌پی‌آی (API) استفاده از جی‌سان آزمایشی و جعلی می‌باشد، لذا از همین رو بر این شدیم که یک پلت فرم
                    پارسی برای برنامه‌‌نویسان ایرانی بوجود بیاوریم که بتوانند، آزمایشات و تست خودشون رو انجام دهند.

                    و این پروژه اوپن سورس بوده و شما هم می‌توانید در آن مشارکت کنید و حتی سرویس شخصی خودتون رو با توجه
                    به نیاز خود راه اندازی کنید.
                </p>
            </div>
            <div>
                <img src="https://www.nylas.com/wp-content/uploads/JSON_Blog_Hero.png" alt="json">
            </div>
        </div>
    </div>
</section>

<div class="card">
    <div class="tools">
        <div class="circle">
            <span class="red box"></span>
        </div>
        <div class="circle">
            <span class="yellow box"></span>
        </div>
        <div class="circle">
            <span class="green box"></span>
        </div>
    </div>
    <div class="card__content">
        <section>
            <div class="container">
                <h1 class="title">
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
                    <a href="{{route('v1.user',1)}}" target="_blank">
                        {{route('v1.user',1)}}
                    </a>
                </code>

                <hr>
                <h3>
                    API RESTFull posts (نوشته‌ها)
                </h3>
                <code>
                    <a href="{{route('v1.posts')}}" target="_blank">
                        {{route('v1.posts')}}
                    </a>
                </code>
                <h3>
                    API RESTFull post (نوشته)
                </h3>
                <code>
                    <a href="{{route('v1.post',\App\Models\Post::limit(1)->first()->slug)}}" target="_blank">
                        {{route('v1.post',\App\Models\Post::limit(1)->first()->slug)}}
                    </a>
                </code>

                <hr>
                <h3>
                    API RESTFull categories (دسته‌بندی‌ها)
                </h3>
                <code>
                    <a href="{{route('v1.categories')}}" target="_blank">
                        {{route('v1.categories')}}
                    </a>
                </code>
                <h3>
                    API RESTFull category (دسته‌ها)
                </h3>
                <code>
                    <a href="{{route('v1.category',\App\Models\Category::orderByDesc('id')->limit(1)->first()->slug)}}"
                       target="_blank">
                        {{route('v1.category',\App\Models\Category::orderByDesc('id')->limit(1)->first()->slug)}}
                    </a>
                </code>

                <hr>
                <h3>
                    API RESTFull products (محصولات)
                </h3>
                <code>
                    <a href="{{route('v1.products')}}" target="_blank">
                        {{route('v1.products')}}
                    </a>
                </code>
                <h3>
                    API RESTFull product (محصول)
                </h3>
                <code>
                    <a href="{{route('v1.product',\App\Models\Product::limit(1)->first()->slug)}}" target="_blank">
                        {{route('v1.product',\App\Models\Product::limit(1)->first()->slug)}}
                    </a>
                </code>

                <hr>
                <h3>
                    API RESTFull products (دیدگاه‌ها|نظرات)
                </h3>
                <code>
                    <a href="{{route('v1.comments')}}" target="_blank">
                        {{route('v1.comments')}}
                    </a>
                </code>
                <h3>
                    API RESTFull product (دیدگاه|نظر)
                </h3>
                <code>
                    <a href="{{route('v1.comment',\App\Models\Comment::limit(1)->first()->id)}}" target="_blank">
                        {{route('v1.comment',\App\Models\Comment::limit(1)->first()->id)}}
                    </a>
                </code>

            </div>
        </section>
    </div>
</div>


<div class="card">
    <div class="tools">
        <div class="circle">
            <span class="red box"></span>
        </div>
        <div class="circle">
            <span class="yellow box"></span>
        </div>
        <div class="circle">
            <span class="green box"></span>
        </div>
    </div>
    <div class="card__content">
        <section>
            <div class="container">
                <h1 class="title">
                    گزینه‌ها
                </h1>
                <h3>
                    مرتب‌سازی | sort
                </h3>
                <code>
                    <a href="{{route('v1.products')}}?sort=view" target="_blank">
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
                    <a href="{{route('v1.products')}}?sortType=desc" target="_blank">
                        {{route('v1.products')}}?sortType=desc
                    </a>
                </code>

                <hr>

                <h3>
                    محدودیت صفحه
                </h3>
                <code>
                    <a href="{{route('v1.products')}}?limit=45" target="_blank">
                        {{route('v1.products')}}?limit=45
                    </a>
                </code>
                <hr>

                <h3>
                    استفاده هم زمان
                </h3>
                <code>
                    <a href="{{route('v1.products')}}?limit=45&sort=sell_count&sortType=desc" target="_blank">
                        {{route('v1.products')}}?limit=45&sort=sell_count&sortType=desc
                    </a>
                </code>
                <hr>
            </div>
        </section>

    </div>
</div>


</body>
</html>
