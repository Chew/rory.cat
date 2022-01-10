class ApplicationController < ActionController::Base
  # Build sitemap
  def sitemap
    # build sitemap xml based on images
    images = Image.pluck(:id)
    sitemap = Nokogiri::XML::Builder.new do |xml|
      xml.urlset(xmlns: 'http://www.sitemaps.org/schemas/sitemap/0.9') do
        xml.url do
          xml.loc 'https://rory.cat'
          xml.changefreq 'daily'
          xml.priority '1.0'
        end
        images.each do |id|
          xml.url do
            xml.loc "https://rory.cat/id/#{id}"
            xml.lastmod Time.now.utc.iso8601
            xml.changefreq 'daily'
            xml.priority '1.0'
          end
        end
      end
    end

    render xml: sitemap.to_xml, status: 200
  end
end
