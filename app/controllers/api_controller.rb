class ApiController < ApplicationController
  include Response

  def random_image
    image = Image.order(Arel.sql('RAND()')).first
    json_response image, 200
  end

  def image_by_id
    image = Image.find_by(id: params['id'])
    if image.nil?
      json_response({ error: "this rory does not exist" }, 404)
      return
    end
    json_response image, 200
  end
end
