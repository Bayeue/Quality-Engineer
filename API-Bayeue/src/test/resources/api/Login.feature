Feature: As a user I want to see my homepage

  Scenario: As a user, I want to login with valid data
    Given  I set Post Api Login endpoint
    When  I send Post Login http request
    Then I receive valid Login http response code 200
    And I receive valid data to login