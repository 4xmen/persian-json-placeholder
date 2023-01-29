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
        code{
            direction: ltr;
            background: #e0e0e0;
            padding: 12px;
            display: block;
        }
        code a{
            color: red;
            text-decoration: none;
            margin: 8px 0;
        }
        .container{
            max-width: 95%;
            margin: 2em auto;
        }
        header svg {
            position: absolute !important;
            bottom: 0;
            background-color: transparent !important;
        }

        header .description {
            text-align: center;
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            z-index: 99;
        }
        header p , header h1 {
            color: #fff;
        }
        .title{
            margin: 25px 0;
            font-size: 2rem;
        }
        .card {
            width: 95%;
            margin: 25px auto;
            padding: 15px;
            background-color: #011522;
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

    </style>
</head>
<body>
<header>
    <div class="description">
        <img style="width: 250px;" src="{{asset('assets/xstack.png')}}" alt="">
        <h1>
            Json placeholder persian - پارسی - فارسی
        </h1>
        <p>
            سرویس فعلی برای آزمایش api بوجود آمده است تا برنامه‌نویسان گرامی برای تست برنامه های خود از api به صورت json
            استفاده کنند و زبان آن پارسی می‌باشد
        </p>
    </div>
    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" style="margin:auto;background:#f5f5f5;display:block;z-index:1;position:relative" width="1366" height="661" preserveAspectRatio="xMidYMid" viewBox="0 0 1366 661">
        <g transform="translate(683,330.5) scale(-1,-1) translate(-683,-330.5)"><linearGradient id="lg-0.895630377801761" x1="0" x2="1" y1="0" y2="0">
                <stop stop-color="#e50914" offset="0"></stop>
                <stop stop-color="#221f1f" offset="1"></stop>
            </linearGradient><path d="" fill="url(#lg-0.895630377801761)" opacity="0.36">
                <animate attributeName="d" dur="9.090909090909092s" repeatCount="indefinite" keyTimes="0;0.333;0.667;1" calcMode="spline" keySplines="0.5 0 0.5 1;0.5 0 0.5 1;0.5 0 0.5 1" begin="0s" values="M0 0L 0 616.0117448675909Q 136.6 623.2299010489415  273.2 572.6052457162517T 546.4 455.5295314532172T 819.6 369.99604749355336T 1092.8 338.9414966624705T 1366 289.4605246806019L 1366 0 Z;M0 0L 0 587.4748912950965Q 136.6 521.4690472188117  273.2 492.21173942950645T 546.4 510.30228086503627T 819.6 370.3296944091511T 1092.8 355.65006484292405T 1366 240.3898611511084L 1366 0 Z;M0 0L 0 554.964649516463Q 136.6 561.3717478019962  273.2 520.4364512338994T 546.4 504.5287962570842T 819.6 434.63650576670966T 1092.8 296.6989027766478T 1366 301.7191577527983L 1366 0 Z;M0 0L 0 616.0117448675909Q 136.6 623.2299010489415  273.2 572.6052457162517T 546.4 455.5295314532172T 819.6 369.99604749355336T 1092.8 338.9414966624705T 1366 289.4605246806019L 1366 0 Z"></animate>
            </path><path d="" fill="url(#lg-0.895630377801761)" opacity="0.36">
                <animate attributeName="d" dur="9.090909090909092s" repeatCount="indefinite" keyTimes="0;0.333;0.667;1" calcMode="spline" keySplines="0.5 0 0.5 1;0.5 0 0.5 1;0.5 0 0.5 1" begin="-1.8181818181818183s" values="M0 0L 0 587.5986772793332Q 136.6 570.1817336395774  273.2 549.3913376342589T 546.4 519.8220847377188T 819.6 439.62918566910963T 1092.8 350.6128849425336T 1366 230.00527420242133L 1366 0 Z;M0 0L 0 620.6668868969638Q 136.6 525.003094730455  273.2 499.3977216303945T 546.4 505.94493228752145T 819.6 468.94572920716416T 1092.8 370.31988518248716T 1366 267.8072190126423L 1366 0 Z;M0 0L 0 563.8943012572545Q 136.6 619.6157971724338  273.2 588.1798961068725T 546.4 503.40873516608025T 819.6 356.8221734156119T 1092.8 315.3479246196325T 1366 321.7181275472819L 1366 0 Z;M0 0L 0 587.5986772793332Q 136.6 570.1817336395774  273.2 549.3913376342589T 546.4 519.8220847377188T 819.6 439.62918566910963T 1092.8 350.6128849425336T 1366 230.00527420242133L 1366 0 Z"></animate>
            </path><path d="" fill="url(#lg-0.895630377801761)" opacity="0.36">
                <animate attributeName="d" dur="9.090909090909092s" repeatCount="indefinite" keyTimes="0;0.333;0.667;1" calcMode="spline" keySplines="0.5 0 0.5 1;0.5 0 0.5 1;0.5 0 0.5 1" begin="-3.6363636363636367s" values="M0 0L 0 639.8535195075846Q 136.6 618.9826659545392  273.2 580.8903307783013T 546.4 471.2635786640349T 819.6 465.59958732901697T 1092.8 405.5720005302434T 1366 306.7841877803726L 1366 0 Z;M0 0L 0 653.791099807293Q 136.6 621.4236386897396  273.2 571.815811292694T 546.4 523.476729359758T 819.6 380.8683712391589T 1092.8 427.22873942400486T 1366 265.4616546112105L 1366 0 Z;M0 0L 0 588.5077843030824Q 136.6 615.230047756007  273.2 575.2378345281122T 546.4 429.00282488035947T 819.6 414.23015521922383T 1092.8 311.62656424868976T 1366 205.5646116594438L 1366 0 Z;M0 0L 0 639.8535195075846Q 136.6 618.9826659545392  273.2 580.8903307783013T 546.4 471.2635786640349T 819.6 465.59958732901697T 1092.8 405.5720005302434T 1366 306.7841877803726L 1366 0 Z"></animate>
            </path><path d="" fill="url(#lg-0.895630377801761)" opacity="0.36">
                <animate attributeName="d" dur="9.090909090909092s" repeatCount="indefinite" keyTimes="0;0.333;0.667;1" calcMode="spline" keySplines="0.5 0 0.5 1;0.5 0 0.5 1;0.5 0 0.5 1" begin="-5.454545454545455s" values="M0 0L 0 570.8488596181816Q 136.6 571.6645770742495  273.2 555.2631565194612T 546.4 439.33884261013714T 819.6 375.98096127741553T 1092.8 364.7523941502424T 1366 292.40161004188843L 1366 0 Z;M0 0L 0 614.7454372078261Q 136.6 539.4939454446842  273.2 508.0679126582524T 546.4 427.5980907437983T 819.6 448.72128240045197T 1092.8 389.08914123055854T 1366 257.62716922655164L 1366 0 Z;M0 0L 0 572.0620126836357Q 136.6 599.6880258477648  273.2 563.6602999306743T 546.4 505.7566276343235T 819.6 466.33647783662377T 1092.8 390.7026082645681T 1366 258.38935858802694L 1366 0 Z;M0 0L 0 570.8488596181816Q 136.6 571.6645770742495  273.2 555.2631565194612T 546.4 439.33884261013714T 819.6 375.98096127741553T 1092.8 364.7523941502424T 1366 292.40161004188843L 1366 0 Z"></animate>
            </path><path d="" fill="url(#lg-0.895630377801761)" opacity="0.36">
                <animate attributeName="d" dur="9.090909090909092s" repeatCount="indefinite" keyTimes="0;0.333;0.667;1" calcMode="spline" keySplines="0.5 0 0.5 1;0.5 0 0.5 1;0.5 0 0.5 1" begin="-7.272727272727273s" values="M0 0L 0 610.6242378801486Q 136.6 523.2155731523778  273.2 493.38340247477885T 546.4 438.45469983378825T 819.6 464.5046070988052T 1092.8 362.7870403360494T 1366 364.9518912925132L 1366 0 Z;M0 0L 0 536.6245870627561Q 136.6 569.8404720819797  273.2 530.0658889603849T 546.4 418.7782752351736T 819.6 361.8029098068135T 1092.8 374.7409368931043T 1366 271.547505494493L 1366 0 Z;M0 0L 0 604.7003869583525Q 136.6 569.6372502228961  273.2 537.0826285646377T 546.4 465.44168931121686T 819.6 434.38707355337016T 1092.8 394.24055164710467T 1366 273.72236992849764L 1366 0 Z;M0 0L 0 610.6242378801486Q 136.6 523.2155731523778  273.2 493.38340247477885T 546.4 438.45469983378825T 819.6 464.5046070988052T 1092.8 362.7870403360494T 1366 364.9518912925132L 1366 0 Z"></animate>
            </path></g>
    </svg>
</header>
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
