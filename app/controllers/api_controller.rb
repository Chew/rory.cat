class ApiController < ApplicationController
  include Response
  skip_before_action :verify_authenticity_token
  before_action :set_cors

  # Allow API to be accessed from any origin
  def set_cors
    response.headers['Access-Control-Allow-Origin'] = '*'
  end

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

    json_response({success: true}, 200)
  end
end
