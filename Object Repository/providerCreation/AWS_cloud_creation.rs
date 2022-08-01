<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AWS_cloud_creation</name>
   <tag></tag>
   <elementGuidId>7b50e9eb-2c86-41c0-8f62-64dfcc533bcb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;provider_type_id\&quot;: 1,\n  \&quot;name\&quot;: \&quot;Test408\&quot;,\n  \&quot;description\&quot;: \&quot;Testing\&quot;,\n  \&quot;access_type_id\&quot;: 2,\n  \&quot;access_key\&quot;: \&quot;AKIASWXVFHNHQXZEUPGO\&quot;,\n  \&quot;secret_key\&quot;: \&quot;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\&quot;,\n  \&quot;default_region_id\&quot;: 7\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>14340d96-b507-4e4b-9e92-967042a1a026</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>461d9dfe5cc5367307ebb055941291e127c2b9c58401d1bfd89251913613412f</value>
      <webElementGuid>fb8523bc-1430-442e-abf3-f2662e7d6799</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-workspace-id</name>
      <type>Main</type>
      <value>62e77b440bcb6a68a900cd70</value>
      <webElementGuid>41a6a838-deec-4740-9ab2-a6c5b8c3a396</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>path</name>
      <type>Main</type>
      <value>/api/admin/provider</value>
      <webElementGuid>55289d2c-b247-41b1-8a3e-30f305fae2b2</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://sandbox.ozone.one/api/admin/provider</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
