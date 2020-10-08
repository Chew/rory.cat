class ApiController < ApplicationController
  include Response
  skip_before_action :verify_authenticity_token

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

  def add_rory
    unless request.headers["Authorization"] == Rails.application.credentials.dig(:key)
      json_response({}, 401)
      return
    end

    Image.create(url: params['url'])
  end
end
