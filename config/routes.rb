Rails.application.routes.draw do
  # For details on the DSL available within this file, see https://guides.rubyonrails.org/routing.html

  # Rory viewing
  get '/', to: 'home#index'
  get 'id/:id', to: 'home#by_id'

  # Rory API
  get 'purr', to: 'api#random_image'
  get 'purr/:id', to: 'api#image_by_id'
  post 'new', to: 'api#add_rory'

  # PWA
  get 'offline', to: 'home#offline'

  # Sitemap
  get 'sitemap.xml', to: 'application#sitemap', defaults: { format: 'xml' }
end
