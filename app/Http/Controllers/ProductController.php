<?php

namespace App\Http\Controllers;

use App\Http\Resources\ProductListResource;
use App\Http\Resources\ProductResource;
use App\Models\Product;
use Illuminate\Http\Request;

class ProductController extends Controller
{

    private $sortable = ['name', 'id', 'created_at', 'updated_at', 'view','sell_count'];
    /**
     * Display a listing of the resource.
     *
     * @return \Illuminate\Http\Response
     */
    public function index(Request $request)
    {
        //
        if ($request->has('sort') && in_array($request->sort, $this->sortable)) {
            $sort = $request->sort;
        } else {
            $sort = 'id';
        }
        if ($request->has('sortType') && strtoupper($request->sortType) == 'DESC') {
            $sortType = 'DESC';
        } else {
            $sortType = 'ASC';
        }
        if ($request->has('limit')) {
            $limit = abs((int) $request->limit);
        } else {
            $limit = 30;
        }

        $paginated = Product::orderBy($sort, $sortType)->paginate($limit);
        $paginated->appends($request->input());
        return response()->json([
                'current_page' => $paginated->currentPage(),
                'data' => ProductListResource::collection($paginated),
                'from' => $paginated->firstItem(),
                'per_page' => $paginated->perPage(),
                'total_pages' => $paginated->lastPage(),
                'count' => $paginated->count(),
                'per_page' => $paginated->perPage(),
                'next_page_url' => $paginated->nextPageUrl(),
//                'current_page_url' => $paginated->url(),
                'prev_page_url' => $paginated->previousPageUrl()
            ]
            ,200);
    }

    /**
     * Show the form for creating a new resource.
     *
     * @return \Illuminate\Http\Response
     */
    public function create()
    {
        //
    }

    /**
     * Store a newly created resource in storage.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Illuminate\Http\Response
     */
    public function store(Request $request)
    {
        //
    }

    /**
     * Display the specified resource.
     *
     * @param  \App\Models\Product  $product
     * @return \Illuminate\Http\Response
     */
    public function show(Product $product)
    {
        //
        return response()->json(ProductResource::make($product),200);
    }

    /**
     * Show the form for editing the specified resource.
     *
     * @param  \App\Models\Product  $product
     * @return \Illuminate\Http\Response
     */
    public function edit(Product $product)
    {
        //
    }

    /**
     * Update the specified resource in storage.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \App\Models\Product  $product
     * @return \Illuminate\Http\Response
     */
    public function update(Request $request, Product $product)
    {
        //
    }

    /**
     * Remove the specified resource from storage.
     *
     * @param  \App\Models\Product  $product
     * @return \Illuminate\Http\Response
     */
    public function destroy(Product $product)
    {
        //
    }
}
