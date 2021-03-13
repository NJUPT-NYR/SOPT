Feature: Register a new user and sign in.

  Scenario: Sign up and login.
    Given a new random user allowed
    Then mock_user is not null
    When I sign up as this user
    Then I get a return json
    Then return json is true
    When I login as this user
    Then I get a return json
    Then return json is true

  Scenario: Sign up duplicate user.
    Given a new random user allowed
    Then mock_user is not null
    When I sign up as this user
    Then I get a return json
    Then return json is true
    When I sign up as this user again
    Then I get a return json
    Then return json is false

  Scenario: Sign up banned user.
    Given a new random user banned
    Then mock_user is not null
    When I sign up as this user
    Then I get a return json
    Then return json is false

  Scenario: Sign up and login with wrong password.
    Given a new random user allowed
    Then mock_user is not null
    When I sign up as this user
    Then I get a return json
    Then return json is true
    When I login with wrong password
    Then I get a return json
    Then return json is false

  Scenario: Login an empty user.
    Given a new random user allowed
    Then mock_user is not null
    When I login as this user
    Then I get a return json
    Then return json is false