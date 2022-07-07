Feature: As a user I want to create new Account

  Scenario: As a user, I want to create new account with valid data
    Given  I set Post Api Register endpoint
    When  I send Post Register http request
    Then I receive valid Register http response code 200
    And I receive valid data for detail user