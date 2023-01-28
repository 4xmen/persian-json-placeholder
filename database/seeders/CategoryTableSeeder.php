<?php

namespace Database\Seeders;

use App\Models\Category;
use Illuminate\Database\Console\Seeds\WithoutModelEvents;
use Illuminate\Database\Seeder;

class CategoryTableSeeder extends Seeder
{
    /**
     * Run the database seeds.
     *
     * @return void
     */
    public function run()
    {
        //
        $parents = [
            'کالای دیجیتال',
            'لباس',
            'کیف و کفش',
        ];
        $children = [
          [
              'parent_id' => 1,
              'name' => 'گوشی موبایل'
          ] ,
          [
              'parent_id' => 1,
              'name' => 'تبلت'
          ] ,
          [
              'parent_id' => 1,
              'name' => 'لپ‌تاپ'
          ] ,
          [
              'parent_id' => 1,
              'name' => 'کامپیوتر شخصی'
          ] ,
          [
              'parent_id' => 2,
              'name' => 'پیراهن مردانه'
          ] ,
          [
              'parent_id' => 2,
              'name' => 'شلوار مردانه'
          ] ,
          [
              'parent_id' => 2,
              'name' => 'لباس بچه گانه'
          ] ,
          [
              'parent_id' => 3,
              'name' => 'کیف زنانه'
          ] ,
          [
              'parent_id' => 3,
              'name' => 'کفش زنانه'
          ] ,
          [
              'parent_id' => 3,
              'name' => 'کیف مردانه'
          ] ,
          [
              'parent_id' => 3,
              'name' => 'کفش مردانه'
          ] ,
        ];
        foreach ($parents as $parent) {
            Category::factory()->create([
                'name' => $parent,
            ]);
        }
        foreach ($children as $child) {
            Category::factory()->create([
                'name' => $child['name'],
                'parent_id'=> $child['parent_id']
            ]);
        }
    }
}
