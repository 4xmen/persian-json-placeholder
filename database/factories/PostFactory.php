<?php

namespace Database\Factories;

use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\Post>
 */
class PostFactory extends Factory
{
    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    public function definition()
    {
        return [
            //
            'title' => $this->faker->unique()->realText(120),
            'body' => $this->faker->realText(350),
            'user_id' => User::inRandomOrder()->limit(1)->first()->id,
            'view' => rand(0,77),
        ];
    }
}
