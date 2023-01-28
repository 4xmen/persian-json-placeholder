<?php

namespace App\Http\Resources;

use Illuminate\Http\Resources\Json\JsonResource;

class ProductResource extends JsonResource
{
    /**
     * Transform the resource into an array.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return array|\Illuminate\Contracts\Support\Arrayable|\JsonSerializable
     */
    public function toArray($request)
    {
        $product =  parent::toArray($request);
        $product['image'] = $this->imageUrl();
        $product['thumb'] = $this->thumbUrl();
        $product['images'] = $this->imagesUrl();
        $product['thumbs'] = $this->thumbsUrl();
        $product['category'] = $this->category;
        $product['user'] = $this->user;
        $product['comments'] = $this->comments;
        return $product;
    }
}
