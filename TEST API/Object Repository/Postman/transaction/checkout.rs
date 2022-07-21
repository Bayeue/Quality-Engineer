<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>checkout</name>
   <tag></tag>
   <elementGuidId>3e58efe6-2fa9-4d31-a4ae-dd6f42cf7dae</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NjIsInBob25lIjoiKzYyODk2MzcwNDUyNzAiLCJzdGF0dXMiOnRydWV9.6fzPWUF1xZJAD77dofNu7mpLcZE5JKlIwVnNsBGO43s</value>
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
      &quot;name&quot;: &quot;id_customer&quot;,
      &quot;value&quot;: &quot;089637045270&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;customer_name&quot;,
      &quot;value&quot;: &quot;Muhammad allam&quot;,
      &quot;type&quot;: &quot;Text&quot;,
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
      <webElementGuid>b9e4f2c0-4095-4685-8599-2cca68bde8fb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-callback-token</name>
      <type>Main</type>
      <value>BjVVRO8eKgceve38jmqm6twtK9YLjtAfk7CbJLxfiToTilHX</value>
      <webElementGuid>73b12ab6-c51e-49ed-b9a7-ec5371a5b46f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NjIsInBob25lIjoiKzYyODk2MzcwNDUyNzAiLCJzdGF0dXMiOnRydWV9.6fzPWUF1xZJAD77dofNu7mpLcZE5JKlIwVnNsBGO43s</value>
      <webElementGuid>4d272d96-7be7-4cff-9ed2-17f60d261a03</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://bayeue.thisham.my.id/api/users/checkout/pulsa-axis-50rb</restUrl>
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
      <id>eaf98206-014b-4430-aed6-fd9a31502c5d</id>
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
