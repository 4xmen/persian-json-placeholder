<?php

namespace Database\Factories;

use App\Models\Category;
use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\Product>
 */
class ProductFactory extends Factory
{
    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    public function definition()
    {
        return [
            'name' => $this->faker->realText(60),
            'description' => $this->faker->realText(350),
            'view' => rand(0,999),
            'sell_count' => rand(0,10),
            'user_id' => User::inRandomOrder()->limit(1)->first()->id,
            'category_id' => Category::inRandomOrder()->limit(1)->first(),
        ];
    }

    public function getName($gender = 'female'){
        if ($gender == 'female'){
            return $this->faker->firstNameFemale();
        }else{
            return $this->faker->firstNameMale();
        }
    }
}
