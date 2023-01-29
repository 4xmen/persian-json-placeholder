# <h1 align='center'> Persian Json Placeholder </h1>

<p dir='rtl'> 
 سرویس فعلی برای آزمایش api بوجود آمده است تا برنامه‌نویسان گرامی برای تست برنامه های خود از api به صورت json استفاده کنند و زبان آن پارسی می‌باشد 
 </p>

# <h2 dir='rtl'> RestfulApi چیست ؟ </h2>

<p dir='rtl'>
قبل از به وجود آمدن معماری (REpresentational State Transfer) Rest Api توسعه دهندگان از پروتکل SOAP برای بهبود عملکرد API استفاده می کردند. REST Api توسط دانشمندی به نام روی فیلینگ ارائه شد که یک سبک معماری برای سیستم های ابر رسانه توزیع شده (Distributed Hypermeida Systems) مانند یوتیوب است (این سبک معماری باعث ایجاد روشی می شود که وب باید طبق آن عمل کند). یک Api مطابق با سبک معماری Rest یک Rest Api نامیده می شود. در واقع یک رابط برنامه نویسی کاربردی مطابق با این سبک از معماری است و برای تعامل با وب سرویس های Rest به کار می رود. Rest به معنای انتقال حالت بازنمایی است.
زمانی که درخواست کلاینت از طریق REST API انجام می شود، سرور یک وضعیت را متناسب با درخواست کلاینت نمایش می دهد. این اطلاعات به وسیله پروتکل HTTP با فرمت JSON ارائه می شوند. همانطور که می دانید HTTP (Hypertext Transfer Protocol) یا پروتکل انتقال فرا متن برای انتقال و نمایش داده ها در فضای وب به کار می رود. 
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
