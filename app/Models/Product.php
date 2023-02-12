<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Spatie\Image\Manipulations;
use Spatie\Sluggable\HasSlug;
use Spatie\Sluggable\SlugOptions;
use Spatie\MediaLibrary\MediaCollections\Models\Media;
use Spatie\MediaLibrary\HasMedia;
use Spatie\MediaLibrary\InteractsWithMedia;
/**
 * App\Models\Product
 *
 * @property int $id
 * @property string $name
 * @property string $slug
 * @property string $description
 * @property int $category_id
 * @property int $view
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @method static \Database\Factories\ProductFactory factory(...$parameters)
 * @method static \Illuminate\Database\Eloquent\Builder|Product newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder|Product newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder|Product query()
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereCategoryId($value)
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereName($value)
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereSlug($value)
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereView($value)
 * @mixin \Eloquent
 * @property int $user_id
 * @property int $sell_count
 * @property-read \App\Models\Category $category
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Comment[] $comments
 * @property-read int|null $comments_count
 * @property-read \Spatie\MediaLibrary\MediaCollections\Models\Collections\MediaCollection|Media[] $media
 * @property-read int|null $media_count
 * @property-read \App\Models\User $user
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereSellCount($value)
 * @method static \Illuminate\Database\Eloquent\Builder|Product whereUserId($value)
 */
class Product extends Model implements HasMedia
{
    use HasFactory,HasSlug,InteractsWithMedia;
    public function getSlugOptions() : SlugOptions
    {
        return SlugOptions::create()
            ->generateSlugsFrom('name')
            ->saveSlugsTo('slug')
            ->usingLanguage('fa')
            ->slugsShouldBeNoLongerThan(32);

    }

    public function registerMediaConversions(Media $media = null): void
    {
        $this->addMediaConversion('product-thumb')
            ->width(400)
            ->height(400)
            ->crop(Manipulations::CROP_CENTER, 400, 400)
            ->optimize()
            ->sharpen(10);
        $this->addMediaConversion('product-image')
            ->height(950)
            ->optimize();

    }

    public function getRouteKeyName()
    {
        return 'slug';
    }

    public function user(){
        return $this->belongsTo(User::class);
    }
    public function category(){
        return $this->belongsTo(Category::class);
    }

    public function comments()
    {
        return $this->morphMany(Comment::class, 'commentable');
    }
    public function thumbUrl()
    {
        if ($this->getMedia()->count() > 0) {
            return $this->getMedia()->first()->getUrl('product-thumb');
        } else {
            return asset('/images/logo.png');

        }
    }
    public function imageUrl()
    {
        if ($this->getMedia()->count() > 0) {
            return $this->getMedia()->first()->getUrl('product-image');
        } else {
            return asset('/images/logo.png');

        }
    }
    public function thumbsUrl()
    {

        $items = [];
        foreach ($this->getMedia() as $media) {
            $items[] = $media->getUrl('product-thumb');
        }

        return $items;
    }
    public function imagesUrl()
    {
        $items = [];
        foreach ($this->getMedia() as $media) {
            $items[] = $media->getUrl('product-image');
        }

        return $items;
    }
}
