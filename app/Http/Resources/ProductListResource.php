<?php

namespace App\Http\Resources;

use Illuminate\Http\Resources\Json\JsonResource;

class ProductListResource extends JsonResource
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
        $product['category'] = $this->category;
        $product['user'] = $this->user;
        $product['url'] = route('v1.product',$this->slug);
        return $product;
    }
    public function withResponse($request, $response)
    {
        $jsonResponse = json_decode($response->getContent(), true);
        unset($jsonResponse['links'],$jsonResponse['meta']);
        $response->setContent(json_encode($jsonResponse));
    }
}
