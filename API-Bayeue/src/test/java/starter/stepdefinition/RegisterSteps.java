package starter.stepdefinition;

import io.cucumber.java.en.And;
import io.cucumber.java.en.Given;
import io.cucumber.java.en.Then;
import io.cucumber.java.en.When;
import net.thucydides.core.annotations.Steps;
import starter.register.RegisterPOST;

public class RegisterSteps {

    @Steps
    RegisterPOST post;

    @Given("I set Post Api Register endpoint")
    public void setPostApiRegisterEndpoint() {
        post.setPostApiRegisterEndpoint();

    }
    @When("I send Post Register http request")
    public void sendPostRegisterHttpRequest() {
        post.sendPostRegisterHttpRequest();

    }
    @Then("I receive valid Register http response code 200")
    public void receiveValidRegisterHttpResponseCode200() {
        post.receiveValidRegisterHttpResponseCode200();

    }
    @And("I receive valid data for detail user")
    public void receiveValidDataForDetailUser() {
        post.receiveValidDataForDetailUser();

    }














}

