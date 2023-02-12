<?php

namespace Database\Seeders;

// use Illuminate\Database\Console\Seeds\WithoutModelEvents;
use App\Models\Comment;
use App\Models\Gallery;
use App\Models\Post;
use App\Models\Product;
use Illuminate\Database\Seeder;
use Illuminate\Support\Facades\Storage;

class DatabaseSeeder extends Seeder
{
    /**
     * Seed the application's database.
     *
     * @return void
     */
    public function run()
    {
        Storage::deleteDirectory('public');
        Storage::makeDirectory('public');


        $this->call([
            UserTableSeeder::class,
            CategoryTableSeeder::class,
            PostTableSeeder::class,
            ProductTableSeeder::class,
            GalleryTableSeeder::class,
            CommentTableSeeder::class,
        ]);
    }
}
