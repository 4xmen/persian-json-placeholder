# <h1 align='center'> Persian Json Placeholder </h1>

<p dir='rtl'> 
یکی از ابزار آموزشی برای برنامه‌نویسان Front-end، برنامه‌نویسان موبایل و یادگیری کار با رست‌فول ای‌پی‌آی (API)  استفاده از جی‌سان آزمایشی و جعلی می‌باشد، لذا از همین رو بر این شدیم که یک پلت فرم پارسی برای برنامه‌‌نویسان ایرانی بوجود بیاوریم که بتوانند، آزمایشات و تست خودشون رو انجام دهند.

و این پروژه اوپن سورس بوده و شما هم می‌توانید در آن مشارکت کنید و حتی سرویس شخصی خودتون رو با توجه به نیاز خود راه اندازی کنید. </p>

# <h2 dir='rtl'> RestfulApi چیست ؟ </h2>

<p dir='rtl'>
REST API (همچنین به عنوان RESTful API شناخته می شود) یک رابط برنامه نویسی کاربردی (API یا web API) است که با محدودیت های سبک معماری REST مطابقت دارد و امکان تعامل با سرویس های وب RESTful را فراهم می کند. REST مخفف انتقال حالت نمایشی است و توسط متخصصان کامپیوتر روی فیلدینگ ایجاد شده است. 
</p>

# <h2 dir='rtl'> چرا باید از RestfulAPi استفاده کنیم؟</h2>

<p> 

ممکن است بخواهید برای طراحی یا تست اپلیکیشن یا سایت خود یک سری اطلاعات برای پر کردن دیتابیس داشته باشید . شاید بپرسید این داده ها از کجا دریافت می شوند؟ داده ها از سرور دریافت می شوند. شما به عنوان کلاینت با استفاده از API، درخواست خود را به سرور ارسال می کنید و سرور پاسخ می دهد. داده ها از سمت سرور به صورت HTML برای کاربر ارسال می شوند اما اگر داده ها به صورت ساختیافته و از طریق XML، JSON به کاربر نمایش داده شوند بهتر و منظم تر است. استخراج اطلاعات پروسه پیچیده ای است که به تنهایی از طریق API امکان پذیر نیست و متد ها و توابع زیادی نیاز است. REST API با ایجاد یک مجموعه دیتا ساختیافته مقادیر آن شی را متناسب با درخواست کاربر برای کلاینت ارسال می کند.

</p>

# <h2 dir='rtl'>روش استفاده از این سرویس </h2>

<h4 dir='rtl'> روش اول استفاده از axios </h4>

```
axios
    .get(" https://json.xstack.ir/api/v1/users")
    .then((result) => {
      console.log(result)
    })
    .catch((err) => console.log(err));
```

<h4> روش دوم استفاده از jquery  </h4>

```
$.get( "ajax/test.html", function( data ) {
  $( ".result" ).html( data );
  alert( "Load was performed." );
});
```

# <h2> امکانات این سرویس </h2>


| # | URL                |
| :-------- | :------------------------- |
| `کاربران` |  https://json.xstack.ir/api/v1/users  |
| `کاربر`   |  https://json.xstack.ir/api/v1/user/1  |
| `نوشته‌ها` |  https://json.xstack.ir/api/v1/posts |
| `نوشته`   |  https://json.xstack.ir/api/v1/post/dolt-chyzy-jz-yk-anban-gshadhy-pa  |
| `دسته‌بندی‌ها`   |  https://json.xstack.ir/api/v1/categories  |
| `دسته‌بندی‌`   |   https://json.xstack.ir/api/v1/category/kfsh-mrdanh   |
| `محصولات`   |   https://json.xstack.ir/api/v1/products   |
| `محصول`   |   https://json.xstack.ir/api/v1/product/goshy-mobayl-tktm   |
| `دیدگاه‌ها/نظرات`   |   https://json.xstack.ir/api/v1/comments   |
| `دیدگاه/نظر`   |    https://json.xstack.ir/api/v1/comment/1    |

# <h2> گزینه ها </h2>
| # | Example                |
| :-------- | :------------------------- |
| `مرتب‌سازی / sort ` |   https://json.xstack.ir/api/v1/products?sort=view  |
| ` نوع مرتب‌سازی / sortType `   |   https://json.xstack.ir/api/v1/products?sortType=desc   |
| ` محدودیت صفحه ` |   https://json.xstack.ir/api/v1/products?limit=45  |
| ` استفاده هم زمان `   |   https://json.xstack.ir/api/v1/products?limit=45&sort=sell_count&sortType=desc   |

* برای مرتب سازی مقدار بین asc,desc میباشد
