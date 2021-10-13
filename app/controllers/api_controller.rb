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

  # Adds a Rory to the DB. Chew only!
  def add_rory
    unless request.headers["Authorization"] == Rails.application.credentials[:key]
      return json_response({ success: false, message: 'invalid key' }, 401)
    end

    Aws.config.update(
      region: 'us-east-2',
      credentials: Aws::Credentials.new(
        Rails.application.credentials.dig(:aws, :access_key_id),
        Rails.application.credentials.dig(:aws, :secret_access_key)
      )
    )

    image = RestClient.get(params['rory'])
    extension = ".#{params['rory'].split('.')[-1]}"

    s3 = Aws::S3::Client.new
    type = image.headers[:content_type]
    o = [('a'..'z'), ('A'..'Z'), (0..9)].map(&:to_a).flatten
    string = (0...10).map { o[rand(o.length)] }.join
    name = string + extension.to_s

    s3.put_object(
      bucket: 'rory.cat',
      body: image.body,
      acl: 'public-read',
      key: name,
      content_type: type
    )
    url = "https://i.rory.cat/#{name}"

    rory = Image.create(url: url)

    json_response({ success: true, url: url, id: rory.id }, 200)
  end
end
