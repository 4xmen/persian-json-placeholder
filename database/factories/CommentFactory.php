<?php

namespace Database\Factories;

use App\Models\Post;
use App\Models\Product;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\Comment>
 */
class CommentFactory extends Factory
{
    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    public function definition()
    {
        if (rand(1,4) == 1){
            $model = Post::class;
            $id = Post::inRandomOrder()->limit(1)->first()->id;
        }else{
            $model = Product::class;
            $id = Product::inRandomOrder()->limit(1)->first()->id;
        }
        return [
            'body' => $this->faker->realText(300),
            'commentable_type' => $model,
            'commentable_id' => $id,
        ];
    }
}
