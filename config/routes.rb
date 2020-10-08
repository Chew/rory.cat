Rails.application.routes.draw do
  # For details on the DSL available within this file, see https://guides.rubyonrails.org/routing.html
  get '/', to: 'home#index'
  get 'id/:id', to: 'home#by_id'
  get 'purr', to: 'api#random_image'
  get 'purr/:id', to: 'api#image_by_id'
  post 'new', to: 'api#add_rory'
end
