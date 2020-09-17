class ApiController < ApplicationController
  include Response

  def random_image
    image = Image.order(Arel.sql('RAND()')).first
    json_response image, 200
  end
end
