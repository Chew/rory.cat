class HomeController < ApplicationController
  def index
    @image = Image.order(Arel.sql('RAND()')).first
    @total = Image.count
  end
end
