<?php

namespace Database\Seeders;

use App\Models\Category;
use App\Models\Product;
use Database\Factories\ProductFactory;
use Illuminate\Database\Console\Seeds\WithoutModelEvents;
use Illuminate\Database\Seeder;

class ProductTableSeeder extends Seeder
{
    /**
     * Run the database seeds.
     *
     * @return void
     */
    public function run()
    {
        //

        $cats = Category::whereNotNull('parent_id')->get();
        $fk = new ProductFactory();
        foreach ($cats as $cat) {
            if ( in_array($cat->id,[13,14,8,9]) ){
                $gender = 'male';
            }else{
                $gender  = 'female';
            }
            for ($i = 0; $i < rand(14,29); $i++) {
                Product::factory()->create(
                    [
                        'name' => $cat->name.' ' . $fk->getName($gender) ,
                        'category_id' => $cat->id,
                    ]
                );
                $product = Product::latest()->limit(1)->first();
                for ($j = 0; $j <= rand(1,4); $j++) {
                    $num = rand(1,40);
                    $product->addMedia(__DIR__ . "/img/$num.jpg")
                        ->preservingOriginal()
                        ->toMediaCollection();
                }
            }
        }
    }
}
