class HomeController < ApplicationController
  def index
    @image = Image.order(Arel.sql('RAND()')).first
    @total = Image.count
  end

  def by_id
    @image = Image.find_by(id: params['id'])
    if @image.nil?
      render html: "this rory does not exist"
    end
    @total = Image.count
  end
end
