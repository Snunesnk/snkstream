from selenium import webdriver
from selenium.webdriver.common.keys import Keys

#Open sonarr
driver = webdriver.Firefox()
driver.get("http://localhost:8989/addseries")

#Fill input search with desired film
search_field = driver.find_element_by_class_name("x-series-search")
print(search_field)
# search_field.send_keys("This is a test")

#Close connexion
driver.close()