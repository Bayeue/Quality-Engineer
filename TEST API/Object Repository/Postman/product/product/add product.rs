<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>add product</name>
   <tag></tag>
   <elementGuidId>a0b8aad2-e56e-4632-aa4f-f0ef0bd280e0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6MiwicGhvbmUiOiIrNjI4OTAwMDAwMDAwMDAiLCJzdGF0dXMiOnRydWV9.kZeecXCmggdLl6hdgmO-g20hPt07mrxGOzkVYZLo9vY</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;name&quot;,
      &quot;value&quot;: &quot;Link Aja&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;file&quot;,
      &quot;value&quot;: &quot;C:\\Users\\ikmal\\Documents\\allam\\icon.png&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <webElementGuid>df896c6c-d4d5-4744-b75a-865acb2b61e5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6MiwicGhvbmUiOiIrNjI4OTAwMDAwMDAwMDAiLCJzdGF0dXMiOnRydWV9.kZeecXCmggdLl6hdgmO-g20hPt07mrxGOzkVYZLo9vY</value>
      <webElementGuid>4f5186a7-ecc2-46f6-80dc-1ed281a050ef</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://bayeue.thisham.my.id/api/admin/products/8</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseurl</defaultValue>
      <description></description>
      <id>eb243cef-7cb4-4608-b2f5-96ffe0a83fd7</id>
      <masked>false</masked>
      <name>baseurl</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
